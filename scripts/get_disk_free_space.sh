sys_type=$(uname)

target=$(ipfs repo stat |grep RepoPath |awk '{print $2}')
space_k=
if [ "$sys_type" == "Darwin" ]; then
	space_k=$(df -k $target |grep -v Filesystem |awk '{print $4}')
elif [ "$sys_type" == "Linux" ]; then
	space_k=$(df -k $target |grep -v Filesystem |awk '{print $4}')
else
	echo only support Darwin and Linux
	exit 1
fi
echo $((space_k * 1024))
