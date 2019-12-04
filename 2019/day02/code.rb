class ProgramAlarm1202
  def initialize(input)
    @initial_input = input
    @input = @initial_input.split(',').map(&:to_i)
  end

  def get_program_result
    cursor = 0
    operation_code = @input[cursor]

    while operation_code != 99
      first_position = @input[cursor + 1]
      second_position = @input[cursor + 2]
      output_position = @input[cursor + 3]

      first_number = @input[first_position]
      second_number = @input[second_position]

      if sum? operation_code
        @input[output_position] = first_number + second_number
      elsif multiply? operation_code
        @input[output_position] = first_number * second_number
      elsif abort? operation_code
        puts 'program finished'
        break
      else
        puts 'something went wrong'
        break
      end

      cursor += 4
      operation_code = @input[cursor]
    end

    @input[0]
  end

  def solve_second_part
    @input[1] = 0
    @input[2] = 0

    for i in 0..100 do
      for j in 0..100 do
        @input = @initial_input.split(',').map(&:to_i)
        @input[1] = i.to_i
        @input[2] = j.to_i

        result = get_program_result

        return (100 * i) + j if result == 19690720
      end
    end
  end

  def sum?(operation_code)
    operation_code == 1
  end

  def multiply?(operation_code)
    operation_code == 2
  end

  def abort?(operation_code)
    operation_code == 99
  end
end

file = File.open './input'
input = file.read
program = ProgramAlarm1202.new input
# puts program.get_program_result
puts program.solve_second_part
