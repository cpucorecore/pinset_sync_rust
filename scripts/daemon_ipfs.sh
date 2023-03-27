nohup ipfs daemon &

for((i=0;i<10;i++))
do
pid=$(ps -ef|grep "ipfs daemon" | grep -v "grep" | awk '{print $2}')
if [ -z "$pid" ]; then
	echo ipfs exited
	exit 1
else
  sleep 1
fi

done