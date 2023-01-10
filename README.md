# kmeans-rs

**Dependencies**

- *rand* [0.8.5]
- *image* [0.24.5]

**About**

The code features a simple implementation of a naive kmeans algorithm applied to processing of images. 

Running the main function will cluster the colours of a provided image of the png format and save a version with only k-colours.

**Example**

The process applied to a picture of the gherkin would look like this for a set of values for *k*:

*Original image*



Original            |  k=2
:-------------------------:|:-------------------------:
![Original image](/resources/test.png)  |  ![Image with 2 colours](/resources/clustered_k2.png)

k=3            |  k=5
:-------------------------:|:-------------------------:
![Image with 3 colours](/resources/clustered_k3.png)  |  ![Image with 5 colours](/resources/clustered_k5.png)

k=7            |  k=11
:-------------------------:|:-------------------------:
![Image with 7 colours](/resources/clustered_k7.png)  |  ![Image with 11 colours](/resources/clustered_k11.png)

