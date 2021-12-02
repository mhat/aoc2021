last = 0
incs = 0

File.open("input.txt", "r").each do |line|
    curr = line.chomp.to_i
    if last == 0 
        puts "#{curr} NA"
    else
        if curr > last 
            puts "#{curr} increased"
            incs += 1
        elsif curr == last 
            puts "#{curr} equal"
        else
            puts "#{curr} decreased"
        end
    end
    last = curr
end

puts "increments: #{incs}"