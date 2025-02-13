// This helper decodes the hidden data from a string of variation selectors.
// It mirrors the Rust decoding logic: variation selectors in U+FE00–U+FE0F represent bytes 0–15,
// and those in U+E0100–U+E01EF represent bytes 16–255.
function decodeHiddenMessage(variationSelectors) {
    let bytes = [];
    // Using a for...of loop here iterates over whole code points (even for surrogate pairs).
    for (const char of variationSelectors) {
      const code = char.codePointAt(0);
      if (code >= 0xFE00 && code <= 0xFE0F) {
        bytes.push(code - 0xFE00);
      } else if (code >= 0xE0100 && code <= 0xE01EF) {
        bytes.push(code - 0xE0100 + 16);
      }
    }
    // Assuming the hidden message is ASCII, convert the byte array to a string.
    return String.fromCharCode(...bytes);
  }
  
  // This function walks through all text nodes under the given root
  // and replaces any encoded sequences with a span that has a red border and a tooltip.
  function highlightHiddenMessages(root) {
    const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, null, false);
    let node;
    // Use a regex that matches a “base” character followed immediately by one or more variation selectors.
    // Note: The regex uses the Unicode flag (u) so that the U+E0100–U+E01EF range is interpreted correctly.
    const regex = /(.)([\uFE00-\uFE0F\u{E0100}-\u{E01EF}]+)/gu;
    const nodesToProcess = [];
  
    // Collect text nodes first.
    while ((node = walker.nextNode())) {
      nodesToProcess.push(node);
    }
  
    nodesToProcess.forEach(textNode => {
      let text = textNode.textContent;
      let match;
      let lastIndex = 0;
      const fragment = document.createDocumentFragment();
  
      // Process all encoded sequences in this text node.
      while ((match = regex.exec(text)) !== null) {
        // Append plain text preceding the match.
        if (match.index > lastIndex) {
          fragment.appendChild(document.createTextNode(text.substring(lastIndex, match.index)));
        }
  
        // match[1] is the base character and match[2] is the sequence of variation selectors.
        const baseChar = match[1];
        const variationSelectors = match[2];
        const fullSequence = baseChar + variationSelectors;
        const decoded = decodeHiddenMessage(variationSelectors);
  
        // Create a span wrapping the encoded sequence.
        const span = document.createElement('span');
        span.textContent = fullSequence;
        // Style it with a red border so it stands out.
        span.style.border = "1px solid red";
        span.style.padding = "0 2px";
        // Set the tooltip text to show the hidden message.
        span.title = decoded;
  
        fragment.appendChild(span);
        lastIndex = regex.lastIndex;
      }
  
      // Append any remaining text after the last match.
      if (lastIndex < text.length) {
        fragment.appendChild(document.createTextNode(text.substring(lastIndex)));
      }
  
      // Replace the original text node with the newly built fragment.
      textNode.parentNode.replaceChild(fragment, textNode);
    });
  }
  
  // Run the highlighter on the document body.
  highlightHiddenMessages(document.body);