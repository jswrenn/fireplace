FREQUENCY=0.1
while true;
do
echo $[ RANDOM % 20 ];
sleep $FREQUENCY;
done | ./target/fireplace 'Random Data'
