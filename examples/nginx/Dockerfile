FROM base
COPY /home/wieslaw/RustProjects/container-runtime/ /home
RUN chmod +x /home/even.sh
RUN chmod +x /home/odd.sh
RUN apt update && apt install -y nginx curl
RUN echo "#!/bin/bash\n tail -f /var/log/nginx/*" >> /home/server.init
RUN chmod +x /home/server.init
ENTRYPOINT /etc/init.d/nginx start;
