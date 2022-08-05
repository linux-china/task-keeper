desc "doit task"
task :doit do
  puts "DONE"
end

task :dont do
  Rake::Task[:doit].clear
end
