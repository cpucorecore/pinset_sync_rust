# index
```
http://192.168.0.87:8889/get_state
http://192.168.0.87:8889/space_info
http://192.168.0.87:8889/sync_review 
http://192.168.0.87:8889/start_ipfs
http://192.168.0.87:8889/start_ipfs_cluster 
http://192.168.0.87:8889/sync
http://192.168.0.87:8889/gc 
```

# apis
## get_state
获取当前服务状态，状态定义如下
```rust
pub enum Status {
    Idle,
    Sync(SyncStatus),
    Gc(GcStatus),
}

pub enum SyncStatus {
    GetSyncReview,
    Syncing,
}

pub enum GcStatus {
    ClusterStopped,
    IpfsStopped,
    StateExported,
    GcFinish,
    IpfsStarted,
    Syncing,
    ClusterStarted,
}
```

```sh
curl http://192.168.0.87:8889/get_state
```

resp
```json
{
  "status": "Idle"
}
```

## space_info
获取当前节点的存储空间信息

```sh
curl http://192.168.0.87:8889/space_info
```

resp
```json
{
  "space_pinned": 18854535119,
  "space_used": 52772576014,
  "space_ipfs_total": 644245094400,
  "space_disk_free": 387170160640,
  "pin_percentage": 35
}
```
- space_pinned: ipfs处于pin状态的文件总大小
- space_used: ipfs存储实际占用的磁盘大小
- space_ipfs_total: ipfs配置的存储大小
- space_disk_free: ipfs存储所在磁盘剩余大小
- pin_percentage: space_pinned/space_used，用于衡量gc的必要性

## sync_review
同步ipfs和ipfs cluster的pin集合--预览(不执行同步，只预览)

```sh
http://192.168.0.87:8889/sync_review
```

resp
```json
{
    "cids_to_add": [
        "QmPF2agcKQt6wGCsUBeskX12drDdqfNhfk4a2ZzeCj5H9j",
        "QmPF8SxaMDrKwHyA5DisRibAvpCWTJztSYAQbtkupaB9bL",
        "QmPFCUkhqwyNa9VZi5GmAJjJLfbUGdXkKaBw7fxrSdzGXf",
        "QmPDfRe7WJpcYZXryvFHMuWEF9LYzPSxrj1ZtGaAXNxTjZ",
        "QmPEZWVTTuzmHmmwL37QXWFwBWagXHWrhqpafgPAf25whh"
        
    ],
    "cids_to_rm": [
        "QmPEknf8HGLXE4iwsKoxgLdTFUDwiVcw18otR85RaRE9RM",
        "QmfMWD3XQyGeFWymEkaYo5c8XrmKQmetvcKg1y7JKoY8gF"
    ]
}
```

- cids_to_add: ipfs需要pin的集合
- cids_to_rm: ipfs需要unpin的集合

## sync
同步ipfs和ipfs cluster的pin集合

```sh
http://192.168.0.87:8889/sync
```

resp
```json
{
  "add_result": {
    "QmPF2agcKQt6wGCsUBeskX12drDdqfNhfk4a2ZzeCj5H9j": true,
    "QmPF8SxaMDrKwHyA5DisRibAvpCWTJztSYAQbtkupaB9bL": true,
    "QmPFCUkhqwyNa9VZi5GmAJjJLfbUGdXkKaBw7fxrSdzGXf": true,
    "QmPDfRe7WJpcYZXryvFHMuWEF9LYzPSxrj1ZtGaAXNxTjZ": true,
    "QmPEZWVTTuzmHmmwL37QXWFwBWagXHWrhqpafgPAf25whh": false
  },
  "rm_result": {
    "QmPEknf8HGLXE4iwsKoxgLdTFUDwiVcw18otR85RaRE9RM": true,
    "QmfMWD3XQyGeFWymEkaYo5c8XrmKQmetvcKg1y7JKoY8gF": true
  }
}
```
- add_result: pin结果
- rm_result: unpin结果
- true: 成功/false: 失败

## start_ipfs
启动ipfs，会检查是否已经启动
```sh
curl http://127.0.0.1:8889/start_ipfs
```

resp
```
ipfs already running
```

## start_ipfs_cluster
启动start_ipfs_cluster，会检查是否已经启动
```sh
curl http://127.0.0.1:8889/start_ipfs_cluster
```

resp
```
cluster already running
```

## gc
执行gc

```sh
curl http://127.0.0.1:8889/gc
```

resp
```json
{
  "err_msg": "ok",
  "before_gc": {
    "space_pinned": 0,
    "space_used": 1320415,
    "space_ipfs_total": 20000000000,
    "space_disk_free": 76554387456,
    "pin_percentage": 0
  },
  "after_gc": {
    "space_pinned": 0,
    "space_used": 1318569,
    "space_ipfs_total": 20000000000,
    "space_disk_free": 76553068544,
    "pin_percentage": 0
  }
}
```

- err_msg: "ok"表示成功，其他表示失败原因
- before_gc: gc前空间信息
- after_gc: gc后空间信息
