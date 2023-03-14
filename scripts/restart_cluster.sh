d=`date "+%Y_%m_%dT%H:%M:%S"`
bash stop_cluster.sh && sleep 3 && mv nohup.out nohup.out.$d && bash daemon_cluster.sh
echo -n new pid:
ps -ef|grep ipfs-cluster-ser |grep -v grep |awk '{print $2}'