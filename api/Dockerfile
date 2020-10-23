# Use the official lightweight Node.js 12 image.
# https://hub.docker.com/_/node
FROM node:12.18-alpine

WORKDIR /opt/server

COPY package.json ./
COPY package-lock.json ./

# Clean, fast install from package-lock.json file excluding dev dependencies.
RUN npm ci --only=production

COPY . .

CMD ["npm", "start"]