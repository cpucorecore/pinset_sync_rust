nohup ipfs-cluster-service --loglevel info daemon &

for((i=0;i<10;i++))
do
pid=$(ps -ef|grep "ipfs-cluster-service" | grep -v "grep" | awk '{print $2}')
if [ -z "$pid" ]; then
	echo ipfs cluster exited
	exit 1
else
  echo ipfs cluster alive
  sleep 1
fi

done