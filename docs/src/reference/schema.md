The STREM tool currently accepts a strict data format to support spatial and temporal reasoning and querying over perception datastreams.

## Overview

The schema is structured as a [JSON](https://www.json.org/json-en.html) object organized into the minimally relevant information to provide searching capabilities over the perception data.


!!! example
	 
	 This example highlights a valid, STREM-accepted, format of a JSON object of one frame with two samples from two different channels.
	 
	 ```json
	 {
	   "version": "0.1.1",
	   "frames": [
	     {
	       "index": 0,
	       "samples": [
	         {
	           "type": "@stremf/sample/detection",
	           "channel": "cam::front",
	           "image": {
	             "path": "train_images/image.png",
	             "dimensions": {
	               "width": 1920,
	               "height": 1080
	             }
	           },
	           "annotations": [
	             {
	               "class": "bus",
	               "score": 0.76,
	               "bbox": {
	                 "type": "@stremf/bbox/obb",
	                 "region": {
	                   "center": {
	                     "x": 1023.2516767573418,
	                     "y": 1679.9571384868343
	                   },
	                   "dimensions": {
	                     "w": 2.6459999999999066,
	                     "h": 18.94800000000005
	                   },
	                   "rotation": -0.28776980428620613
	                 }
	               }
	             },
	 	    {
	               "class": "pedestrian",
	               "score": 0.82,
	               "bbox": {
	                 "type": "@stremf/bbox/aabb",
	                 "region": {
	                   "center": {
	                     "x": 1023.2516767573418,
	                     "y": 1679.9571384868343
	                   },
	                   "dimensions": {
	                     "w": 2.6459999999999066,
	                     "h": 18.94800000000005
	                   },
	                 }
	               }
	             }
	           ]
	         }
	       ]
	     }
	   ]
	 }
	 ```

## Schema

The schema is separated into four distinct JSON object literals.

```json
"version": str,
"frames": [ frame ]
```


```json title="frame"
"index": int,
"samples": [ sample ]
```


```json title="sample"
"type": "@stremf/sample/detection",
"channel": str,
"image": {
    "path": str//(1)!,
    "dimensions": {
	    "width": int,
	    "height": int
    }
},
"annotations": [ annotation ]
```

1. The `path` to the associated image is relative to the JSON file location. If no file provided, it is relative to the working directory from which the `strem` command was invoked.

```json title="annotation"
"class": str,
"score": float,
"bbox": aabb | obb
```


```json title="aabb"
"type": "@stremf/bbox/aabb",
"region": {
	"center": {
		"x": float,//(1)!
		"y": float//(2)!
	}
	"dimensions": {
		"w": float,
		"h": float
	}
},
```

1. The `x` coordinate represents the horizontal center position of a bounding box.
2. The `y` coordinate represents the vertical center position of a bounding box.


```json title="obb"
"type": "@stremf/bbox/obb",
"region": {
	"center": {
		"x": float//(1)!,
		"y": float//(2)!
	}
	"dimensions": {
		"w": float,
		"h": float
	},
	"rotation": float//(3)!
},
```

1. The `x` coordinate represents the horizontal center position of a bounding box.
2. The `y` coordinate represents the vertical center position of a bounding box.
3. The units of the `rotation` field must be in radians.
