#FROM ruby:alpine
FROM ruby:3.2.6-alpine3.20

WORKDIR /data

# Update system dependencies and install build tools
RUN apk update && apk upgrade && apk add --no-cache \
    build-base \
    git \
    libxml2-dev \
    libxslt-dev \
    postgresql-dev \
    nodejs \
    tzdata

ADD ./ruby /data
RUN bundle install

CMD ["rake", "xdr:update"]
