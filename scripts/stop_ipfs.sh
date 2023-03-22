pid=`ps -ef|grep "ipfs daemon" | grep -v "grep" | awk '{print $2}'`
if [ -z $pid ]; then
	echo already stoped
else
	echo pid $pid alive, to kill it
	ipfs_repo_lock=`lsof -p $pid |grep lock |awk '{print $9}'`
	kill $pid

	while true;
	do
	  if [ -e ipfs_repo_lock ]; then
	    echo kill sended, wait ipfs shutdown
	    sleep 1
	  else
	    break
	  fi
	done
fi