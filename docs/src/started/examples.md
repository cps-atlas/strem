# Examples

## Autonomous Vehicle Perception Dataset

### Download the Dataset

For this example, we will be using the Woven Planet Perception dataset. This dataset may be downloaded [here](https://woven.toyota/common/assets/data/3d-object-detection-one_scene.tar).

After downloading, unpack the archive.

### Fix the Dataset

Within this dataset, the JSON files, unfortunately, do not follow the JSON standard and use "NaN" in-place of the widely accepted "null" type. Fortunately, this error only occurs within a single file from the dataset. You may fix this file by running the following command:

```bash
jq . train_data/sample_data.json > tmp.json && mv tmp.json sample_data.json
```

### Convert the Dataset

To convert the dataset, we must first install a converter tool. To install the converter tool, run the following command:

```bash
cargo install --git https://github.com/cps-atlas/stremf.git
```

After installation, run the following command to convert the dataset:

```bash
stremf --debug --schema="nuscenes" --input="train_data/" "./"
```


### Offline Search

!!! example "Find all instances where the bounding box of the car is greater than 300K pixels and is oriented to the left side of the image."

    ```bash
    strem --channel="cam::front" "[E(v := [:car:])(@area(v) > 300000.0 & @x(v) < 700.0)]" ./*.json
    ```

!!! example "Find all instances where the distance between all cars and a pedestrian is greater than 500.0 pixels."

    ```bash
    strem --channel="cam::front" "[A(v := [:car:])(@dist(v, [:pedestrian:]) > 500.0)]" ./*.json
    ```

!!! example "Find all instances where the distance between all cars and all pedestrians is greater than 500.0 pixels."

    ```bash
    strem --channel="cam::front" "[A(v := [:car:], p := [:pedestrian:])(@dist(v, p) > 500.0)]" ./*.json
    ```

### Online Search

For online searching and monitoring, it is necessary to add the `--online` flag to properly select the online matching algorithm capable of matching in real-time. Therefore, for example, such a command would look like:

!!! example "Find all instances where the distance between all cars and all pedestrians is greater than 500.0 pixels."

    ```bash
    strem --online --channel="cam::front" "[A(v := [:car:], p := [:pedestrian:])(@dist(v, p) > 500.0)]" ./*.json
    ```
