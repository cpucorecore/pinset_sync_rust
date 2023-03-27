pid=`ps -ef|grep ipfs-cluster-service |grep -v grep |awk '{print $2}'`
if [ -z $pid ]; then
	echo already stoped
else
	echo pid "$pid" alive, to kill it
	cluster_file_lock=$(lsof -p $pid |grep lock |awk '{print $9}')
	kill "$pid"

	while true;
	do
	  if [ -e "$cluster_file_lock" ]; then
	    echo kill sended, wait cluster shutdown
	    sleep 1
	  else
	    break
	  fi
	done
fi
