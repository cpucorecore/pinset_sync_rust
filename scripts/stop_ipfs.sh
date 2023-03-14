pid=`ps -ef|grep "ipfs daemon" | grep -v "grep" | awk '{print $2}'`
if [ ! -z $pid ]; then
  kill -9 $pid
else
  echo not running
fi