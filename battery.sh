FREQUENCY=0.1
while true;
do
acpi -b | awk -F " |%" '{print $4}'
done | ./target/fireplace 'Random Data'
