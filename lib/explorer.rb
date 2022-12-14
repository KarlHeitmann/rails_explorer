require_relative 'explorer/node'

module Explorer
  # Helper class to deal with IO
  class IOUtils
    def getCmdData(cmd)
      io = IO.popen(cmd)
      data = io.read
      io.close
      # raise 'it failed!' unless $?.exitstatus == 0
      data
    end
  end

  # Main Nodes for explorer class
  class Nodes
    UP_ARROW = "\e[A"
    RIGHT_ARROW = "\e[C"
    DOWN_ARROW = "\e[B"
    LEFT_ARROW = "\e[D"

    def self.tty_screen_width
      TTY::Screen.columns
    end

    def screen_width
      @screen_width ||= self.class.tty_screen_width
    end

    def parse_data(lines, aux = [], grouped_lines = [])
      lines.each do |line_string|
        line = JSON.parse line_string
        if %w[begin context match].include? line['type']
          aux << line
        elsif line['type'] == 'summary'
          @summary = line['data']
        elsif line['type'] == 'end'
          aux << line
          grouped_lines << aux
          aux = []
        else
          1 / 0
        end
      end
      grouped_lines
    end

    def initialize(explorer_data:, previous_data:)
      @io = IOUtils.new
      @explorer_data = explorer_data
      @previous_data = previous_data
      @filter = ''
      @prompt = TTY::Prompt.new
      lines = rg_launch
      grouped_lines = parse_data(lines)
      @nodes = grouped_lines.map { Node.new(_1, explorer_data: explorer_data) }
    end

    def print_previous_data
      text_previous_data = ''
      @previous_data.each do |data|
        text_previous_data += <<~TEXT_PREVIOUS_DATA
          search_term: #{green(data[:search_term])}
          path: #{green(data[:path])}

        TEXT_PREVIOUS_DATA
      end
      title = { top_left: 'Previous data stack' }
      print TTY::Box.frame(top: 0, title: title) { text_previous_data }
    end

    def summary_box_and_filenames_choices
      # TODO: Adjust size of the box accordingly
      text_to_display = ''
      choices = []
      nodes = @nodes.filter { _1.name_file.include? @filter }
      nodes.each_with_index do |node, i|
        file_name = node.name_file.gsub(@explorer_data[:prefix], '')
        text_to_display << file_name << "\n"
        choices << { name: file_name, value: i + 0 }
      end
      choices << { name: 'Quit', value: 'q' }
      title = { top_left: @explorer_data[:search_term], bottom_right: @explorer_data[:path] }
      [TTY::Box.frame(top: 0, width: 30, height: choices.size + 2, title: title) { text_to_display }, choices]
    end

    def filenames_filtered
      @nodes
        .filter { _1.name_file.include? @filter }
        .map { "#{_1.name_file.gsub(@explorer_data[:prefix], '')}:#{_1.matches_count}" }
    end

    def summary_box
      # TODO: Adjust size of the box accordingly
      text_to_display = ''
      nodes = @nodes.filter { _1.name_file.include? @filter }
      nodes.each do |node|
        file_name = node.name_file.gsub(@explorer_data[:prefix], '')
        text_to_display << "#{file_name}:#{node.matches_count}\n"
      end
      title = { top_left: @explorer_data[:search_term], bottom_right: @explorer_data[:path] }
      TTY::Box.frame(top: 0, height: nodes.size + 2, title: title) { text_to_display }
    end

    def individual_action(option)
      complete_file_name = @nodes[option].name_file
      file_name = complete_file_name.split('/')[-1]
      return unless (file_name[0] == '_') && (file_name[-3..] == 'erb')

      plugin_rails_command = "render.*#{file_name.split('.').first[1..]}" # TODO: Here we need to apply a plugin
      choices = [
        { name: "Spawn explorer to search >>\"#{plugin_rails_command}\"<<", value: 1 },
        { name: 'Return', value: 'q' },
      ]
      option = @prompt.enum_select("INDIVIDUAL ACTION #{complete_file_name}", choices)  
      return if option == 'q'

      explorer_child_data = @explorer_data
      explorer_child_data[:search_term] = plugin_rails_command
      new_data = {search_term: plugin_rails_command, path: @explorer_data[:path]}
      pd = @previous_data.clone
      pd << new_data
      explorer_child = Nodes.new(explorer_data: explorer_child_data, previous_data: pd)
      explorer_child.menu
    end

    def rg_launch_raw(search_term, list_files)
      # cmd = "rg #{@explorer_data[:search_term]} --json #{@explorer_data[:path]}".split
      # cmd = "rg #{search_term} #{list_files.reduce('') { "#{_1} #{_2}" }} -A #{@explorer_data[:spanlines]} -B #{@explorer_data[:spanlines]} --pretty".split
      cmd = "rg #{search_term} #{list_files.reduce('') { "#{_1} #{_2}" }} --pretty".split
      @io.getCmdData(cmd)
    end

    def rg_launch
      # cmd = "rg #{@explorer_data[:search_term]} --json #{@explorer_data[:path]}".split
      if @explorer_data[:subcommand_files].nil? # TODO # XXX Change nil check by initializing subcommand_files key as empty array []
        cmd = "rg #{@explorer_data[:search_term]} #{@explorer_data[:path]} -A #{@explorer_data[:spanlines]} -B #{@explorer_data[:spanlines]} --json".split
      else
        # cmd = "rg #{@explorer_data[:search_term]} #{@nodes.reduce('') { _1 + ' ' + _2.name_file }} -A #{@explorer_data[:spanlines]} -B #{@explorer_data[:spanlines]} --json".split
        cmd = "rg #{@explorer_data[:search_term]} #{@explorer_data[:subcommand_files].reduce('') { "#{_1} #{_2}" }} -A #{@explorer_data[:spanlines]} -B #{@explorer_data[:spanlines]} --json".split
      end
      @io.getCmdData(cmd).split("\n")
    end

    def explore_menu(choices, selected = nil)
      unless selected.nil?
        loop do
          c = @prompt.keypress('> Arrows to span more/less lines, any other key to continue:')
          case c
          when UP_ARROW
            @nodes[selected].span_lines += 1 if @nodes[selected].span_lines < @explorer_data[:spanlines].to_i
          when RIGHT_ARROW
            @nodes[selected].span_lines += 1 if @nodes[selected].span_lines < @explorer_data[:spanlines].to_i
          when DOWN_ARROW
            @nodes[selected].span_lines -= 1 if @nodes[selected].span_lines.positive?
          when LEFT_ARROW
            @nodes[selected].span_lines -= 1 if @nodes[selected].span_lines.positive?
          else
            break
          end

          draw_file_matches(choices, selected)
        end
      end

      @prompt.enum_select('EXPLORE', choices + [{ name: 'Quit', value: 'q' }])
    end

    def draw_file_matches(choices, option)
      clear_screen
      text_detail = @nodes[option].matches
      max_height = [choices.size, text_detail.count("\n")].max
      title = { top_left: " #{@nodes[option].name_file}:#{@nodes[option].matches_count} " }
      detail = TTY::Box.frame(top: 0, width: screen_width, height: max_height + 2, title: title) { text_detail }
      print detail
    end

    def explore
      choices = filenames_filtered.map.with_index { { name: _1, value: _2 } }
      clear_screen
      option = nil
      loop do
        print_previous_data
        option = explore_menu(choices, option)
        # clear_screen
        break if option == 'q'

        draw_file_matches(choices, option)

        individual_action(option) unless @explorer_data[:quick]
      end
    end

    def input_filter
      @filter = ''
      box = nil
      loop do
        box = summary_box
        clear_screen
        print box
        c = @prompt.keypress("> #{@filter}:")
        break if c == "\r"

        if c == "\u007F" # This is a backspace
          @filter = @filter[0...-1]
          next
        end
        @filter << c
      end
      box
    end

    def menu
      box = summary_box
      loop do
        clear_screen
        print_previous_data
        print box
        choices = [
          { name: 'Explore', value: 1 },
          { name: 'Filter', value: 2 },
          { name: 'Run ripgrep over matched files (apply filter)', value: 3 },
          { name: 'Run ripgrep raw over matches', value: 4 },
          { name: 'Quit', value: 'q' }
        ]

        total_matches = @nodes.filter { _1.name_file.include? @filter }.count
        option = @prompt.enum_select("MAIN MENU\nTotal files with matches: #{total_matches}", choices)
        case option
        when 1
          explore
        when 2
          box = input_filter # Alternative use tty-prompt filter: https://github.com/piotrmurach/tty-prompt#2627-filter
        when 3
          pd = @previous_data
          # search_pattern = @prompt.ask('Enter search pattern')
          explorer_child_data = @explorer_data
          r = @prompt.ask('Enter search pattern').chomp
          puts r.inspect
          explorer_child_data[:search_term] = r
          explorer_child_data[:subcommand_files] = @nodes.filter { _1.name_file.include? @filter }.map { _1.name_file }
          # total_matches = @nodes.filter { _1.name_file.include? @filter }.count
          explorer_child = Nodes.new(explorer_data: explorer_child_data, previous_data: pd)
          # explorer_child.subcommand
          explorer_child.menu
        when 4
          search_term = @prompt.ask('Enter search term')
          puts rg_launch_raw(search_term, @nodes.filter { _1.name_file.include? @filter }.map { _1.name_file })
          @prompt.ask('Press any key to continue')
        else
          break
        end
      end
    end

    def subcommand
    end

    def to_s
      @nodes.reduce('') { _1 + _2.to_s }
    end
  end
end

