use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL do site que contém o preço do Bitcoin
    let url = "https://www.okx.com/pt-br/price/bitcoin-btc";

    // Faz uma solicitação GET para o site
    let resp = reqwest::get(url).await?.text().await?;

    // Analisa o texto da resposta como HTML
    let document = Html::parse_document(&resp);

    // Cria um seletor para encontrar o elemento que contém o preço do Bitcoin
    let selector = Selector::parse(".price-last").unwrap();

    // Itera sobre os elementos encontrados
    for element in document.select(&selector) {
        // Extrai o texto do elemento selecionado
        let preco = element.text().collect::<Vec<_>>();

        // Imprime o preço do Bitcoin
        println!("Preço do Bitcoin: {:?}", preco);
    }

    Ok(())
}

