#!/bin/sh
protoc protos/user.proto --js_out=apps/mobile/clients/protos/
protoc protos/user.proto --python_out=clients/python/
