<template>
  <v-container class="fill-height">
    <v-responsive class="align-centerfill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />

      <div class="text-center">
        <div class="text-body-2 font-weight-light mb-n1">Office of Enterprise Data & Analytics</div>

        <h1 class="text-h2 font-weight-bold">Fuzzy Joiner</h1>
      </div>

      <div class="py-4" />

      <v-row>
        <v-col cols="12">
          <v-card class="pt-5 px-4" rounded="lg" variant="tonal">
            <v-row>
              <v-col>Step 1: Select the two files you would like to join</v-col>
            </v-row>
            <v-row>
              <v-col cols="6">
                <v-file-input label="File 1" variant="outlined" @change="handleFile1Change" />
              </v-col>
              <v-col cols="6">
                <v-file-input label="File 2" variant="outlined" @change="handleFile2Change" />
              </v-col>
            </v-row>
            <v-row>
              <v-col>Step 2: Map the join columns</v-col>
            </v-row>
            <v-row>
              <v-col cols="6">
                <v-select v-model="selected1[0]" :items="file1Headers" density="compact" variant="outlined"
                  label="Compact" append-icon="mdi-arrow-left"></v-select>
                <v-select v-model="selected1[1]" :items="file1Headers" density="compact" variant="outlined"
                  label="Compact" append-icon="mdi-arrow-left"></v-select>
                <v-select v-model="selected1[2]" :items="file1Headers" density="compact" variant="outlined"
                  label="Compact" append-icon="mdi-arrow-left"></v-select>
                <v-select v-model="selected1[3]" :items="file1Headers" density="compact" variant="outlined"
                  label="Compact" append-icon="mdi-arrow-left"></v-select>
              </v-col>
              <v-col cols="6">
                <v-select v-model="selected2[0]" :items="file2Headers" density="compact" variant="outlined"
                  label="Compact" prepend-icon="mdi-arrow-right"></v-select>
                <v-select v-model="selected2[1]" :items="file2Headers" density="compact" variant="outlined"
                  label="Compact" prepend-icon="mdi-arrow-right"></v-select>
                <v-select v-model="selected2[2]" :items="file2Headers" density="compact" variant="outlined"
                  label="Compact" prepend-icon="mdi-arrow-right"></v-select>
                <v-select v-model="selected2[3]" :items="file2Headers" density="compact" variant="outlined"
                  label="Compact" prepend-icon="mdi-arrow-right"></v-select>
              </v-col>
            </v-row>
            <v-row>
              <v-col>Step 3: Set the tolerance level</v-col>
            </v-row>
            <v-row>
              <v-col cols="12">
                <v-slider v-model="tolerance" thumb-label="always" :max="1.0" :min="0.0" :step="0.1"></v-slider>
              </v-col cols="12">
            </v-row>
            <v-row class="d-flex justify-center align-center pb-6">
              <v-col cols="auto">
                <v-btn prepend-icon="mdi-rocket" size="x-large" rounded="xl" @click="handleGoClick">
                  <template v-slot:prepend>
                    <v-icon color="success"></v-icon>
                  </template>
                  GO!
                </v-btn>
              </v-col>
            </v-row>
          </v-card>
        </v-col>
      </v-row>
      <v-overlay :model-value="isLoading" class="align-center justify-center">
        <v-progress-circular :size="70" :width="7" indeterminate>
        </v-progress-circular>
      </v-overlay>
    </v-responsive>
  </v-container>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { parse } from 'papaparse'
import init, { BKTreeWrapper } from '../../bktree/pkg/bktree.js'

const tolerance = ref(0.3)
const file1 = ref(null)
const file2 = ref(null)
const file1Headers = ref([])
const file2Headers = ref([])
const selected1 = ref([])
const selected2 = ref([])
const isLoading = ref(false)

const wasmModule = ref(null)
let tree = null

onMounted(async () => {
  wasmModule.value = await init()
})

const handleFileChange = (file, headers) => {
  const reader = new FileReader()
  reader.onload = (e) => {
    parse(e.target.result, {
      header: true,
      preview: 1,
      step: (results, parser) => {
        headers.value = Object.keys(results.data)
        parser.abort()
      },
      error: (error, file) => {
        console.error('Error while reading file:', error)
      }
    })
  }
  reader.readAsText(file)
}

const handleFile1Change = (e) => {
  file1.value = e.target.files[0]
  handleFileChange(file1.value, file1Headers)
}

const handleFile2Change = (e) => {
  file2.value = e.target.files[0]
  handleFileChange(file2.value, file2Headers)
}

const buildTree = async () => {
  if (!file1.value || !file2.value) {
    console.error('Both files must be uploaded before building the tree')
    return
  }
  const largerFile = file1.value.size > file2.value.size ? file1.value : file2.value;
  const largerFileSelection = file1.value.size > file2.value.size ? selected1.value : selected2.value;

  const reader = new FileReader();
  reader.onload = (e) => {
    tree = BKTreeWrapper.new();
    parse(e.target.result, {
      header: true,
      step: (results) => {
        // Check if the line is blank
        if (Object.values(results.data).every(value => value === '')) {
          return;
        }
        let key = largerFileSelection.map(header => results.data[header]).join(' ')
        key = key.replace(/\W/g, '').toUpperCase();
        const values = Object.values(results.data)
        tree.insert(key, values)
      },
      complete: () => {
        isLoading.value = false
      },
      error: (error, file) => {
        console.error('Error while reading file:', error)
      }
    })
  }
  reader.readAsText(largerFile);
  await new Promise(resolve => reader.onloadend = resolve);
}

const handleGoClick = async () => {
  try {
    isLoading.value = true;
    await buildTree();
    searchTree();
  } catch (error) {
    console.error(error);
  }
}

const searchTree = () => {
  const smallerFile = file1.value.size <= file2.value.size ? file1.value : file2.value;
  const smallerFileSelection = file1.value.size <= file2.value.size ? selected1.value : selected2.value;

  let csvData = 'data:text/csv;charset=utf-8,';
  csvData += smallerFile === file1.value ? file1Headers.value.map(header => `${header}1`).join(',') + ',' : file2Headers.value.map(header => `${header}2`).join(',') + ',';
  csvData += smallerFile === file1.value ? file2Headers.value.map(header => `${header}2`).join(',') + '\n' : file1Headers.value.map(header => `${header}1`).join(',') + '\n';

  const reader = new FileReader();
  reader.onload = (e) => {
    parse(e.target.result, {
      header: true,
      step: (results) => {
        if (Object.values(results.data).every(value => value === '')) {
          return;
        }
        let key = smallerFileSelection.map(header => results.data[header]).join(' ')
        key = key.replace(/\W/g, '').toUpperCase();
        const searchResults = tree.search(key, tolerance.value)
        if (searchResults.length > 0) {
          searchResults.forEach(resultArray => {
            const row = Object.values(results.data).map(value => `${value}`.replace(/,/g, '')).join(',');
            const resultRow = resultArray.map(value => `${value}`.replace(/,/g, '')).join(','); // Remove all commas from the strings in the array
            csvData += `${row},${resultRow}\n`;
          });
        }
      },
      error: (error, _) => {
        console.error('Error while reading file:', error)
      },
      complete: () => {
        isLoading.value = false
        const encodedUri = encodeURI(csvData);
        const link = document.createElement('a');
        link.setAttribute('href', encodedUri);
        link.setAttribute('download', 'search_results.csv');
        document.body.appendChild(link);
        link.click();
      }
    })
  }
  reader.readAsText(smallerFile);
}
</script>