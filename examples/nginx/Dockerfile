FROM base
RUN apt update && apt install -y nginx curl
RUN echo "#!/bin/bash\n tail -f /var/log/nginx/*" >> /init
RUN chmod +x /init
ENTRYPOINT /etc/init.d/nginx start;
# Start the container with the following command
# container-runtime start {container_name} {image_name} {network [host,none]} init 
