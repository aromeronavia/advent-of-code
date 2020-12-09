class SumFinder
  TARGET = 2020

  def initialize(input)
    @input = input
  end

  def solve_first_part
    for i in (0..@input.size - 1)
      for j in (0..@input.size - 1)
        if @input[i] + @input[j] == TARGET
          return @input[i] * @input[j]
        end
      end
    end

    0
  end

  def solve_second_part
    for i in (0..@input.size - 1)
      for j in (0..@input.size - 1)
        for k in (0..@input.size - 1)
          if @input[i] + @input[j] + @input[k] == TARGET
            return @input[i] * @input[j] * @input[k]
          end
        end
      end
    end

    0
  end
end

file = File.open('./input')
input = file.readlines.map(&:chomp).map(&:to_i)
runner = SumFinder.new input
puts 'Solution for part one ' + runner.solve_first_part.to_s
puts 'Solution for part two ' + runner.solve_second_part.to_s

# Solution for part one 440979
# Solution for part two 82498112
