FROM base
COPY nodejs-server app
RUN apt update
RUN apt install -y nodejs npm
RUN echo "#!/bin/bash\n cd app && node index.js" > /init
RUN chmod +x /init
# Run the container with the command /init
