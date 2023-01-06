# kmeans-rs
Simple implementation of the kmeans clustering algorithm in rust. 
Mainly for the benefit of my own learning.

**Dependencies**

- rand [0.8.5]
- image [0.24.5]

**About**

The code features a simple implementation of a naive kmean clustering algorithm applied to do some processing of images. 

Running the main function will cluster the colours of a provided image of the png format and save a version with only k-colours.

Example:

*Original image*
![Original image](/resources/gherkin.png)

**k = 11**
![Image with 11 colours](/resources/clustered_k11.png)

**k = 7**
![Image with 7 colours](/resources/clustered_k7.png)

**k = 3**
![Image with 3 colours](/resources/clustered_k3.png)

**k = 2**
![Image with 2 colours](/resources/clustered_k2.png)
