#script to append new line of randomly generated data every 5 seconds for 30 seconds, simulates live data updates
import random
import time
from datetime import datetime
import csv

datafile = "./data/mockdata.csv"
endtime = time.time() + 30
id = 10

def generate(id):
    time = datetime.now().isoformat()
    lat = round(random.uniform(-90, 90), 2) 
    lon = round(random.uniform(-180, 180), 2)  
    alt = random.uniform(100, 1000)  
    accel_x = random.uniform(-10, 10) 
    accel_y = random.uniform(-10, 10)
    accel_z = random.uniform(-10, 10) 
    gyro_x = random.uniform(-500, 500) 
    gyro_y = random.uniform(-500, 500)
    gyro_z = random.uniform(-500, 500)
    dac_1 = random.uniform(0, 5)  
    dac_2 = random.uniform(0, 5)  
    dac_3 = random.uniform(0, 5) 
    dac_4 = random.uniform(0, 5) 

    return [id, time, lat, lon, alt, accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z, dac_1, dac_2, dac_3, dac_4]



while time.time() < endtime:
    with open(datafile, 'a', newline='') as csvfile:
        writer = csv.writer(csvfile)
        
        writer.writerow(generate(id))
        id += 1
    #csvfile.close()
    time.sleep(5)

print("timed out")