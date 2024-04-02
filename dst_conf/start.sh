./dontstarve_dedicated_server_nullrenderer -console \
-cluster ${active} -shard Master

./dontstarve_dedicated_server_nullrenderer -console \
-cluster ${active} -shard Caves

# ${active} 文件夹的名字，不需要加绝对路径前缀
# 绝对的应该是persist_path 吧，懒得求证了
# Master 里面的文件夹的名字

./dontstarve_dedicated_server_nullrenderer -console -cluster Cluster_2 -shard Master
./dontstarve_dedicated_server_nullrenderer -console -cluster Cluster_2 -shard Caves