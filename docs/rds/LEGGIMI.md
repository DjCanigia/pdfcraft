# PdfCraft 0.14.0 — impostazioni di stampa per Windows RDS

Pacchetto di modifica **non ufficiale**, preparato il 27 ottobre 2026. Versione installata
confermata dall'utente: 0.14.0. Nessuna modifica al server RDS, alle stampanti o al registro
di questa macchina è stata eseguita. Nessun fork o workflow remoto è stato creato.

## Cosa cambia

Ogni apertura del dialogo Stampa su Windows parte con **scala di grigi attiva** e
**fronte-retro sul lato lungo**. L'utente può scegliere colori, stampa solo fronte,
lato corto o impostazioni della stampante per quel lavoro. Riaprendo Stampa vengono
riproposti i valori predefiniti. Le altre scelte mantengono il comportamento originale.

L'amministratore può impostare i valori per tutte le sessioni del singolo host RDS in
`HKLM\SOFTWARE\PdfCraft\PrintDefaults`, vista registro **64 bit**, valori REG_DWORD:

| Nome | Valori | Se assente/non valido |
|---|---|---|
| Grayscale | 0 colori; 1 scala di grigi | 1 |
| Duplex | 0 default stampante; 1 solo fronte; 2 lato lungo; 3 lato corto | 2 |

Il programma legge questa chiave, senza scriverla e senza richiedere privilegi amministrativi.
La chiave è nuova: **PdfCraft ufficiale 0.14.0 non la legge**. Importare il file .reg senza
installare la build modificata non risolve il problema. Non occorre importarlo per avere
grigi/lato lungo nella build modificata: sono già i valori di ripiego.

I file .reg sono esempi da rivedere e usare solo su un ambiente autorizzato. Nessuno è stato
importato. La chiave va gestita per ogni host della farm, per esempio con Group Policy
Preferences e ACL standard HKLM; non concedere scrittura agli utenti ordinari.

## Repository e versione esatti

- [Repository ufficiale](https://github.com/storytold/pdfcraft).
- [Rilascio verificato v0.14.0](https://github.com/storytold/pdfcraft/releases/tag/v0.14.0).
- Base della patch: `5a78b223871add8e3883856205cc653d741e0e94` (tag v0.14.0).
- Il main osservato era `1dd2737b47a9c1c9562f0af563053d1bf510e4bf`, anch'esso dichiara
  0.14.0, ma ha modifiche successive: la patch è per il **tag**, non per main.
- La stampa del tag delega a craft-libs/craft-print, revisione
  `666c358065261a2816b376fa3bb10538dbf53040`.

In quella revisione `ticket.rs` invia esplicitamente `PageOutputColor=Color` con la casella
grigi disattivata, oppure `Monochrome` quando è attiva. `Duplex::LongEdge` diventa
`JobDuplexAllDocumentsContiguously=TwoSidedLongEdge`. Il trasporto usa Windows.Data.Pdf e XPS,
in processo; non è la stampa di un'applicazione Electron o un comando PowerShell.
I valori scelti nel dialogo raggiungono lo spooler tramite `PrintDraft::job`.

Il main esaminato usa una revisione diversa di craft-print che non forza Color quando grigi
è disattivato. Non confondere questo cambiamento successivo con la build 0.14.0 installata.

Sono state controllate anche le modifiche upstream: la [PR 2103](https://github.com/storytold/pdfcraft/pull/2103)
è ancora aperta e propone di ricordare le preferenze personali tra riavvii; la [PR 2584](https://github.com/storytold/pdfcraft/pull/2584)
è stata integrata e lascia il duplex alla stampante. Questo pacchetto affronta invece i
valori iniziali centralizzati per host RDS, riproposti per ogni nuovo dialogo. Nessuna PR
concorrente è stata aperta e il lavoro non dipende dall'integrazione della PR 2103.

## Applicare la patch su una macchina di sviluppo

Non eseguire questi passaggi sul server di produzione. Usare un percorso corto, ad esempio
`C:\Build`, su una macchina Windows x64 di sviluppo con spazio sufficiente per Rust.

```powershell
git clone --branch v0.14.0 --single-branch https://github.com/storytold/pdfcraft.git
cd pdfcraft
git switch -c fix/windows-rds-print-defaults
git rev-parse HEAD
# Deve essere 5a78b223871add8e3883856205cc653d741e0e94
git apply --check C:\percorso\pdfcraft-0.14.0-rds-defaults.patch
git apply C:\percorso\pdfcraft-0.14.0-rds-defaults.patch
git diff --check
```

La patch contiene i sorgenti modificati, i test Rust e una nota tecnica in
`docs/windows-rds-print-defaults.md`. La logica del backend e il Cargo.lock restano quelli
del tag. I widget con i marchi ArtCraft sono sostituiti da testo neutro che identifica
la build personalizzata, come richiesto dalla licenza dei marchi upstream. Non distribuire
la cartella `docs/brand` come parte di un sorgente modificato. Conservare licenze e avvisi.

## Compilare e creare il pacchetto

Prerequisiti da installare sulla macchina di sviluppo: Visual Studio Build Tools con
Desktop development with C++, strumenti MSVC x64, Windows SDK, Rust tramite rustup,
Git e PowerShell 7. Aprire Developer PowerShell / x64 Native Tools e preparare:

```powershell
rustup toolchain install 1.95.0 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.95.0 x86_64-pc-windows-msvc wasm32-unknown-unknown
```

Per i font giapponesi delle build ufficiali, input opzionale ma raccomandato:

```powershell
git clone https://github.com/storytold/craft-fonts.git ..\craft-fonts
git -C ..\craft-fonts checkout 3d9ab3a132ed0eff457e424226a435a901da30d3
$env:CRAFT_FONTS_DIR = (Resolve-Path ..\craft-fonts).Path
```

Eseguire lo script incluso in **una PowerShell dedicata**: configura variabili di build
per quella sessione e formatta i sorgenti con rustfmt prima di verificare e compilare.

```powershell
& C:\percorso\Build-Windows.ps1 -Source C:\Build\pdfcraft
```

Lo script verifica base e strumenti, esegue test del crate UI, clippy, controlli di livelli,
asset e wasm, poi compila app e CLI x64. Si ferma al primo errore. Usa due processi di
compilazione per i controlli principali. In CI impostare un timeout di circa 45 minuti;
se si blocca una build locale, interromperla e analizzare il log prima di riprovare.
Non si applica ai test l'etichetta "passati" finché i comandi non terminano con successo.

Per MSI e ZIP portabile serve anche .NET SDK e WiX 5.0.2. Nella radice del checkout:

```powershell
dotnet tool install wix --tool-path .tools --version 5.0.2
& C:\percorso\Build-Windows.ps1 -Source C:\Build\pdfcraft -Package
```

Lo script di packaging upstream scarica i modelli OCR con i controlli di integrità previsti
dal progetto. App e CLI saranno sotto `target-rds\x86_64-pc-windows-msvc\release`;
i pacchetti sotto `dist\rds`. Non viene eseguita alcuna installazione.
Per versioni complete dei comandi di packaging vedere `packaging/windows/package.ps1`.
Le credenziali di firma upstream non sono disponibili: una build locale sarà senza firma,
salvo configurare una propria firma. Non è un rilascio firmato dal produttore.

L'etichetta dei pacchetti è `0.14.0-rds.1`, mentre la versione numerica MSI rimane **0.14.0**
per il comportamento del packaging upstream. Non presumere un aggiornamento in-place
dell'MSI ufficiale: provarne installazione/ripristino su VM. Anche la versione base Cargo
resta 0.14.0; la scritta custom RDS e l'hash della patch distinguono la personalizzazione.

## Collaudo richiesto prima dell'uso in produzione

1. Su VM o host RDS di prova, aprire un PDF sintetico di almeno due pagine con elementi
   colorati. Verificare grigi attivo e lato lungo all'apertura.
2. Stampare e verificare **fisicamente** monocromia e rilegatura sul lato lungo.
   Ripetere sui driver reali, inclusi stampanti reindirizzate ed Easy Print se utilizzati.
3. Scegliere colori e solo fronte per un lavoro: verificare ticket in coda e fogli.
   Riaprire Stampa: devono tornare grigi/lato lungo; ripetere dopo Cancel e dopo riavvio.
4. Verificare due utenti RDS e le architetture effettivamente distribuite. Dopo avere
   salvato eventuali valori esistenti, provare le alternative .reg e il ripristino.
   Le modifiche si leggono alla prossima apertura del dialogo, anche in processi già aperti.
5. Verificare stampante senza duplex, booklet e cut-and-stack. Booklet mantiene la propria
   scelta di rilegatura; cut-and-stack richiede Off e altrimenti mostra l'errore originale.
6. Verificare connessione/disconnessione RDS, stampante predefinita, annullamento lavori,
   installazione/disinstallazione, firma e regole AppLocker sul sistema di prova.

La revisione del backend 0.14.0 non garantisce che ogni driver conservi grigi e duplex
nel PrintTicket validato: questa patch non aggiunge controlli o fallback a quel livello.
Non garantisce quindi risultato su ogni stampante senza collaudo.
La normale esportazione vettoriale "Salva come PDF" non converte automaticamente i colori
in grigio: qui si impostano principalmente opzioni del driver, non una conversione del PDF.
Le impostazioni predefinite di `doc_print` senza GUI non cambiano.

## Stato reale delle verifiche

- Verificati repository, tag, manifest e codice effettivo di dialogo e backend Windows.
- API di registro confrontate con i sorgenti winsafe 0.0.29.
- Test Rust aggiunti per decodifica, override, riapertura, controlli UI e layout speciali.
- `cargo test` non è partito: cargo/rustc e MSVC non sono presenti sulla macchina di lavoro.
- Non eseguiti: compilazione Rust, clippy/rustfmt, controlli layer/asset/wasm, test Rust,
  visualizzazione del dialogo modificato, stampa reale, creazione o installazione MSI.
- **Nessun EXE/MSI compilato o certificato viene fornito in questo pacchetto.**

I controlli statici effettivamente eseguiti sono registrati in `VERIFICA.txt`.

## Ripristino

Prima di importare valori su una macchina autorizzata, salvare la chiave se esiste,
usando la vista 64 bit. Per tornare ai valori precedenti, ripristinare quel backup;
se i due valori prima non esistevano, usare l'esempio `remove-example-values.reg`.
Rimuovere i valori nella build modificata fa tornare **grigi/lato lungo**, non i vecchi
default ufficiali. Per il comportamento originale impostare Grayscale=0 e Duplex=0,
oppure ripristinare l'eseguibile ufficiale tramite il piano di rollback approvato.
Nessun rollback è stato eseguito automaticamente.
