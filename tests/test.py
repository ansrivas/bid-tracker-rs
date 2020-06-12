import httpx

print("New bid")
resp = httpx.post("http://localhost:3000/api/v1/bids", 
json={"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d-79fe-4b14-9c19-35a69a89219a", "timestamp": 1212321, "amount":32})                                                                                                                                                  
print(resp.json() )
print("===============================")
print("New bid")
resp = httpx.post("http://localhost:3000/api/v1/bids", 
json={"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d-79fe-4b14-9c19-35a69a89219a", "timestamp": 1212321, "amount":33})                                                                                                                                            
print(resp.json() )
print("===============================")

print("New bid")
resp = httpx.post("http://localhost:3000/api/v1/bids", 
json={"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d-79fe-4b14-9c19-35a69a89219a", "timestamp": 1212321, "amount":99}) 
print(resp.json() )
print("===============================")

print("New bid")
resp = httpx.post("http://localhost:3000/api/v1/bids", 
json={"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d-79fe-4b14-9c19-35a69a89219a", "timestamp": 1212321, "amount":121}) 
print(resp.json() )
print("===============================")
 

print("Trying to bid on not-allowed UUIDs") 
resp = httpx.post("http://localhost:3000/api/v1/bids", 
json={"useruuid":"ae8f7716-867b-4479-b455-c5769e7475ba", "itemuuid": "b2f9ee6d89219a", "timestamp": 1212321, "amount":99}) 
print(resp.status_code )
print(resp.json() )
print("===============================")




print("Get all bids for a given itemuuid")
resp = httpx.get("http://localhost:3000/api/v1/bids/b2f9ee6d-79fe-4b14-9c19-35a69a89219a")  
print(resp.text)
print(resp.json() )
print("===============================")


print("Get all bids by a given useruuid")
resp = httpx.get("http://localhost:3000/api/v1/users/ae8f7716-867b-4479-b455-c5769e7475ba/bids")  
print(resp.json() )
print("===============================")


print("Get the winning bid for a uuid") 
resp = httpx.get("http://localhost:3000/api/v1/bids/b2f9ee6d-79fe-4b14-9c19-35a69a89219a/winning") 
print(resp.json() )
print("===============================")