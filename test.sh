FREQUENCY=0.1
while true;
do
echo $[ RANDOM % 2 ];
sleep $FREQUENCY;
done | ./target/fireplace -t 'Test Title'
