FROM node:lts as frontend
ARG BASE_URL /
WORKDIR /app/web-ui
COPY web-ui/package*.json /app/web-ui/
RUN npm ci
COPY web-ui/ /app/web-ui/
RUN npm run build

FROM thesilican/httpd
COPY --from=frontend /app/web-ui/dist/ /public/
