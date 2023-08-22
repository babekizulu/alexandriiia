# ALEXANDRiiiA

### @author: Lwandle Babekizulu Dlamini

## Contact: 
@babekizulu on X, Threads & Instagram

### @contributors: 
* Lwandle Babekizulu Dlamini,

## What is ALEXANDRiiiA?

### ALEXANDRiiiA, inspired by the Library of Alexandria, will render a simulated reconstruction of historical moments by combining multi agent task simulation, XR and ML models trained on historical data. The goal is to create immersive environments in which users can view interactions between AI agents trained on data which would help faciliate dialogue based on historical context and create multiple possible scenarios of how an historical event might have occurred.

# Timeline: (2023 - 2037)

# The Problem we’re solving for:
* A more interactive way to do historical analysis
* A more interactive way to teach history
* Facilitates the creation of a central cloud of historical data
* Facilitates the creation of a central cloud for  3d assets and environmental reconstructions of historical scenes.
* Gives an opportunity for historical archive libraries which have not yet gone fully digital, to contribute their collection to    our digital ecosystem where it can be kept safely
# Potential Roadblocks:
* Which historical narrative is the most authentic? How do we decide this? Do we offer multiple possible narratives which are 
  accepted within academia?
* Safeguarding historical data. Do we need multiple data centers distributed around the world?
* Sourcing etiquette. How do we acquire, store and utilize historical data in a culturally respectful, ethical and safe way?
  Politically motivated roadblocks.




# Tech Stack:
* Tensorflow / Pytortch for MATSim & Period LLM’s
* Unreal Engine (c++) -> Web Assembly  / Rust WebGPU -> Web Assembly
* Blender or other 3D software for modeling buildings & individuals
* Generative 3D based on map and topology data of area, for environment modeling
* WebXR for all XR functionality
* Backend: Rust
* Front-End GUI: Rust
* Database: PostgreSQL
* Vector Database: Milvus
* Google Sheets for initial curation and data cleaning


# Technical Requirements:
* Multi Agent Task Simulation trained on Historical Data
* Historical Data Collection
* ML Model trained on Historical Data
* XR, VR Functionality and 3D Graphics for reconstructing historical locations and people

# Step by step process:
* Start with one small town, learn and collect it’s recent history
* Extract data points from this history and store it on a spreadsheet
* When we’ve created a dataset from all of the data points, clean the dataset
* Collect map data of the town and it’s buildings
* Either manually 3D model the town in blender or use AI to generate a 3D model based on the towns map.
* Store spreadsheet dataset on a vector db
* Do some machine learning magic with the dataset
* Share the dataset on Kaggle
* Build front-end UI of the app with Rust Dioxus
* Build backend of the app with Rust
* Use WebGPU and Rust to load 3D models from Amazon S3
* We’ll start by creating an LLM based on our historical dataset, and each part of the town will automatically begin to send chat 
  styled information when you hover over a specific area. No characters to start with, just an empty ghost town which is prompted 
  by you hovering over its buildings and automatically predicts that you are requesting information on this area.
