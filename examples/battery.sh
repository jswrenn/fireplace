while true;
do
    acpi -b | awk -F " |%" '{print $4}'
    sleep 1.0
done | cargo run -- -f -l 0 -u 100 -t 'Battery Remaining'
