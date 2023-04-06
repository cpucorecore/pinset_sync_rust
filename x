{
                  "ID": "service_name",
                  "Name": "service_name",
                  "Address": "service_address",
                  "Port": 9094,
                  "Meta": {},
                  "Check": {
          "name": "cluster-check",
          "http": "http://192.168.0.85:19094/id",
          "interval": "10s",
          "timeout": "3s"
        }
        }
