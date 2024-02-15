FROM base
COPY /home/wieslaw/RustProjects/container-runtime/ /home
RUN chmod +x /home/even.sh
RUN chmod +x /home/odd.sh
