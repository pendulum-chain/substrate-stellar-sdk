FROM ruby:alpine

WORKDIR /data

RUN apk add git

ADD ./ruby /data
RUN bundle install

CMD ["rake", "xdr:update"]
