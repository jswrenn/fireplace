FREQUENCY=0.1
T=0.0
while true;
do
    echo "scale=5;s($T)" | bc -l;
    T=$T+0.1;
    sleep $FREQUENCY;
done | ./target/fireplace -t 'Sin'
