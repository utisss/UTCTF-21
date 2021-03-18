FROM python:rc-alpine

WORKDIR /usr/src/app
RUN apk update && apk add python3-dev

RUN pip install --upgrade pip
COPY /requirements.txt /usr/src/app/requirements.txt
RUN pip install -r requirements.txt

COPY . /usr/src/app

EXPOSE 5000

ENTRYPOINT ["python3"]

CMD ["/usr/src/app/app.py"]
