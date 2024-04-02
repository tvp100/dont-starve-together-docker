FROM debian:buster-slim
LABEL Name=dontstarvetogetherdocker Version=amd64_0.0.1 

ENV dst_user="dst"
ENV dst_group="dst_group"
ENV user_path="/home/${dst_user}"
ENV steam_path="${user_path}/steam"
ENV dst_path="${user_path}/server"

ARG steam_url="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz"


# Copy aliyun deb registry
COPY deb.registry.aliyun /etc/apt/sources.list

# install dependencies
RUN dpkg --add-architecture i386 \
    && apt-get update -y \
    && apt-get install -y lib32gcc1 lib32stdc++6 libcurl4-gnutls-dev:i386 wget tar \
    && apt-get install -y screen \
    && apt-get autoremove -y \
    && apt-get clean -y

# 安装测试工具
RUN apt-get install -y vim

# 安装ps工具查看进程
RUN apt-get install -y procps


# add dst user and group
RUN groupadd "${dst_group}" \
    && useradd -g "${dst_group}" "${dst_user}"

# create folder and install steam
RUN mkdir -p ${dst_path} \
    && mkdir -p ${steam_path}/tmp \
    && wget "${steam_url}" -O ${steam_path}/tmp/steamcmd.tar.gz \
    && tar -xvzf ${steam_path}/tmp/steamcmd.tar.gz -C ${steam_path} \
    && rm -rf ${steam_path}/tmp

# Copy dst script
COPY update_dst ${steam_path}

# install dont starve together server
RUN ${steam_path}/steamcmd.sh +runscript ${steam_path}/update_dst

# chown user
RUN chown -R ${dst_user}.${dst_group} ${user_path}

USER ${dst_user}

CMD [ "bash" ]
