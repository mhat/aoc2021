vals = File.open("input.txt", "r").map do |line|
    line.chomp.to_i
end

sums = []
vals.each_with_index do |val0, idx|
    val1 = vals.size > idx+1 ? vals[idx+1] : 0 
    val2 = vals.size > idx+2 ? vals[idx+2] : 0
    rsum = val0+val1+val2
    puts "rsum=#{rsum} 0=#{val0} 1=#{val1} 2=#{val2}"
    sums << rsum
end

last = 0
curr = 0
incs = 0
sums.each do |curr|
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