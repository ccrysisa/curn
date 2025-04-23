import subprocess

command_template = "sudo ./target/debug/curn --command {} --mount ./ubuntu-fs --uid 0 --add ../lim/tests/:/tmp/lim/ --tool ./curn-tool"
commands = [
    '"/bin/ls"', 
    '"/bin/ls -l"', 
    '"/bin/pwd"', 
    '"/bin/date"', 
    '"/bin/whoami"', 
    '"/bin/echo"', 
    '"/bin/echo hello"', 
    '"/bin/ps"', 
    '"/bin/exit"', 
    '"/bin/mkdir /tmp/lest"', 

    '"/curn/lasm -i /tmp/lim/123.lasm -o /tmp/lest/123.lim"', 
    '"/curn/delasm -i /tmp/lest/123.lim"', 
    '"/curn/lime -i /tmp/lest/123.lim"', 
    '"/bin/rm /tmp/lest/123.lim"', 

    '"/curn/lasm -i /tmp/lim/alloc.lasm -o /tmp/lest/alloc.lim"', 
    '"/curn/delasm -i /tmp/lest/alloc.lim"', 
    '"/curn/lime -i /tmp/lest/alloc.lim"', 
    '"/bin/rm /tmp/lest/alloc.lim"', 

    '"/curn/lasm -i /tmp/lim/e.lasm -o /tmp/lest/e.lim"', 
    '"/curn/delasm -i /tmp/lest/e.lim"', 
    '"/curn/lime -i /tmp/lest/e.lim"', 
    '"/bin/rm /tmp/lest/e.lim"', 

    '"/curn/lasm -i /tmp/lim/f123.lasm -o /tmp/lest/f123.lim"', 
    '"/curn/delasm -i /tmp/lest/f123.lim"', 
    '"/curn/lime -i /tmp/lest/f123.lim"', 
    '"/bin/rm /tmp/lest/f123.lim"', 

    '"/curn/lasm -i /tmp/lim/fib.lasm -o /tmp/lest/fib.lim"', 
    '"/curn/delasm -i /tmp/lest/fib.lim"', 
    '"/curn/lime -i /tmp/lest/fib.lim"', 
    '"/bin/rm /tmp/lest/fib.lim"', 

    '"/curn/lasm -i /tmp/lim/pi.lasm -o /tmp/lest/pi.lim"', 
    '"/curn/delasm -i /tmp/lest/pi.lim"', 
    '"/curn/lime -i /tmp/lest/pi.lim"', 
    '"/bin/rm /tmp/lest/pi.lim"', 

    '"/bin/rmdir /tmp/lest"', 
]

total_commands = 0
passed_commands = 0
failed_commands = 0

for cmd in commands:
    command = command_template.format(cmd)
    
    try:
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        
        if result.returncode == 0:
            print(f"Command {cmd} executed successfully with return code 0.")
            passed_commands += 1
        else:
            print(f"Command {cmd} failed with return code {result.returncode}.")
            failed_commands += 1
    except subprocess.CalledProcessError as e:
        print(f"Command {cmd} failed with return code {e.returncode}.")
        failed_commands += 1
    
    total_commands += 1

print("\nSummary:")
print(f"Total: {total_commands}, Passed: {passed_commands}, Failed: {failed_commands}")
