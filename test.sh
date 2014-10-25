FREQUENCY=0.1
while true;
do
#echo $[ RANDOM % 20 ];
echo '2'
sleep $FREQUENCY;
done | ./target/fireplace 'People in this bed'
