class RocketEquationSolver
  def initialize(input)
    @input = input
  end

  def solve_first_part
    @input.reduce(0) do |total, mass|
      total + get_fuel(mass)
    end
  end

  def solve_second_part
    @input.reduce(0) do |total, mass|
      total_fuel = 0
      new_mass = mass.to_i

      loop do
        total_fuel += get_fuel(new_mass)
        new_mass = get_fuel(new_mass)
        break if get_fuel(new_mass) <= 0
      end

      total + total_fuel
    end
  end

  def get_fuel(mass)
    (mass.to_i / 3) - 2
  end
end

file = File.open('./input')
input = file.readlines.map(&:chomp)
runner = RocketEquationSolver.new input
puts 'Solution for part one ' + runner.solve_first_part.to_s
puts 'Solution for part two ' + runner.solve_second_part.to_s
