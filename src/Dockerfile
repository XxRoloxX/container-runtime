FROM base
COPY /home/wieslaw/RustProjects/container-runtime/ /home
RUN chmod +x /home/even.sh
RUN chmod +x /home/odd.sh
RUN apt update && apt install -y nginx
ENTRYPOINT /etc/init.d/nginx start;
