# wyvern
=============
A command-line image processing tool written in Rust.

This tool has been tested with .png and .jpg files. Support for other formats has not been verified yet!

_This tool is a work in progress._

#### Functions:
- Image downscaling
- Batch downscaling of a directory of images
- Fast k-means clustering approximation to get the 'common colors' of an image
- Edge detection for a given image

#### Examples

##### Downscale a single image
The new resized image will be saved under _\path\to\image_minimized_

    image-resize "\path\to\image"


##### Batch downscale a directory of images
The output of this command will be saved under a new directory _\path\to\directory\resized_images_

    batch-resize "\path\to\directory" png
    

##### Return set of common colors in an image
The output is printed to stdout

    kmeans "\path\to\image"

##### Generate new image highlighting the edges of an image
The new resized image will be saved under _\path\to\image_edges_

    edge-detect "\path\to\image"
