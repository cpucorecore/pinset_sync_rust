pid=$(pgrep pinset_sync_rust)
if [ -z "$pid" ]; then
	echo already stoped
else
	echo pid "$pid" alive, to kill it
	kill "$pid"
fi