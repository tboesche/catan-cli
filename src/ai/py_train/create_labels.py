import os
import pandas as pd
import h5py

def append_to_hdf5(hdf5_file, dataset_name, data):
    dataset = hdf5_file[dataset_name]
    dataset.resize((dataset.shape[0] + data.shape[0]), axis=0)
    dataset[-data.shape[0]:] = data

def process_csv_to_hdf5_incrementally(csv_dir, hdf5_path):
    # Create or open the HDF5 file
    with h5py.File(hdf5_path, 'a') as hdf5_file:
        # Initialize the datasets if they do not exist
        if 'names' not in hdf5_file:
            max_shape = (None,)  # Unlimited size
            hdf5_file.create_dataset('names', shape=(0,), maxshape=max_shape, dtype=h5py.string_dtype(encoding='utf-8'))
            hdf5_file.create_dataset('labels', shape=(0,), maxshape=max_shape, dtype='uint8')
            hdf5_file.create_dataset('features', shape=(0, 0), maxshape=(None, None), dtype='uint16', compression="gzip")
            # hdf5_file.create_dataset('features', shape=(0, 0), maxshape=(None, None), dtype='uint16')
        
        # Iterate over each file in the directory
        for filename in os.listdir(csv_dir):
            if filename.endswith('.csv'):
                file_path = os.path.join(csv_dir, filename)
                base_filename = os.path.splitext(filename)[0]  # Remove the ".csv" extension
                df = pd.read_csv(file_path, header=None)

                names = []
                labels = []
                features = []

                for idx, row in df.iterrows():
                    if row[0] > 1000:
                        break

                    name = f"{base_filename}_{row[0]}"
                    label = row[1]
                    feature = row[2:].to_list()

                    names.append(name.encode('utf-8'))
                    labels.append(label)
                    features.append(feature)
                
                # Convert lists to appropriate formats
                names = pd.Series(names).to_numpy()
                labels = pd.Series(labels).astype('uint8').to_numpy()
                features = pd.DataFrame(features).astype('uint16').to_numpy()

                # Ensure the 'features' dataset can accommodate the new columns if necessary
                if features.shape[1] > hdf5_file['features'].shape[1]:
                    hdf5_file['features'].resize((hdf5_file['features'].shape[0], features.shape[1]))

                # Append data to the HDF5 file
                append_to_hdf5(hdf5_file, 'names', names)
                append_to_hdf5(hdf5_file, 'labels', labels)
                append_to_hdf5(hdf5_file, 'features', features)

# Example usage
csv_directory = 'data/saves/test_hot_encode'
hdf5_file_path = 'data/ai/random_ne_processed/short_data.hdf5'
process_csv_to_hdf5_incrementally(csv_directory, hdf5_file_path)