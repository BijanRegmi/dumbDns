# DumbDNS

Very simple name server implemenation serving only A records. Developed for the purpose of learning dns protocol.

## Testing

- Run the server

```
./run.sh
```

- Send dns query

```
dig bijanregmi.com.np @127.0.0.1
```

## Adding new records

- Follow the format as in `zones/bijanregmi.com.np.zone` file

## Reference

- https://datatracker.ietf.org/doc/html/rfc1035

## Things that could be added

- Caching
- Recursive resolver
- Support for other [record formats](https://datatracker.ietf.org/doc/html/rfc1035#section-3.3)
