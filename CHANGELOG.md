# 🏷️ Atualização 0.9.3

### ✨ Novidades:
- Uso de RWLock para melhorar as operações de leitura e escrita durante a execução do programa.
- Adição da função `PI()` para retornar o valor de PI na biblioteca `matematica`.
- Tratamento de erros movidos para a análise, permitindo que o programa termine mais cedo.
- Adição da função `erro()` para exibir mensagens de erro no console.
- Melhorado a maneira de lidar com o handle do aplicativo e o handle do console.
- Ao tentar utilizar uma função de uma biblioteca não importada, o programa exibirá uma dica de como importar a biblioteca.

### 🐛 Correções:
- Corrigido aviso de erro ao tentar importar uma biblioteca que existe.
- Corrigido erro onde o programa continuava a esperar um evento mesmo após o encerramento da execução.