
# coding: utf-8

# In[1]:

from ctypes import *

runtime_ds = CDLL("../target/debug/libruntime_datastructure.so")


# In[2]:

class C_Label(Structure):
    _fields_ = [("x", c_double),
                ("y", c_double),
                ("t", c_double),
                ("osm_id", c_long),
                ("prio", c_int),
                ("label", c_char_p)]
    
class C_Result(Structure):
    _fields_ = [("size", c_uint64),
                ("data", POINTER(C_Label))]
    
class Label:
    def __init__(self, l):
        self.x = l.x
        self.y = l.y
        self.t = l.t
        
        self.osm_id = l.osm_id
        self.prio = l.prio
        self.label = l.label
        
    def to_string(self):
        return "Label #{}: '{}' at (x: {}, y:{}) with prio {} has t = {}".format(self.osm_id, self.label, self.x, self.y, self.prio, self.t)


# In[3]:

# init
runtime_ds.init.argtypes = [c_char_p]
runtime_ds.init.restype = c_void_p

# is_good
runtime_ds.is_good.argtypes = [c_void_p]
runtime_ds.is_good.restype = c_bool

#get_data
runtime_ds.get_data.argtypes = [c_void_p, c_double, c_double, c_double, c_double, c_double]
runtime_ds.get_data.restype = C_Result


# In[4]:

def to_c_string(s):
    return create_string_buffer(s.encode('utf-8'))


# In[ ]:

ds = runtime_ds.init(to_c_string("../resources/baden-wuerttemberg-latest.osm.pbf.ce"))

print("Initialisieren der Datenstuktur war erfolgreich: {}".format(runtime_ds.is_good(ds)))


# In[6]:

res = runtime_ds.get_data(ds, 2, 8, 9, 53, 54)


# In[8]:

l = list()
for i in range(res.size):
    l.append(Label(res.data[i]))


# In[9]:

for e in l:
    print("{}\n".format(e.to_string()))


# In[10]:

res.data[0].osm_id


# In[ ]:



