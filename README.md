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
![Original image](/resources/gherkin.png)

k=2            |  k=3
:-------------------------:|:-------------------------:
![Image with 2 colours](/resources/clustered_k2.png)  |  ![Image with 3 colours](/resources/clustered_k3.png)

k=7            |  k=11
:-------------------------:|:-------------------------:
![Image with 7 colours](/resources/clustered_k7.png)  |  ![Image with 11 colours](/resources/clustered_k11.png)

