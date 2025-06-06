// JavaScript for Uniswap V4 Core Documentation

document.addEventListener('DOMContentLoaded', function() {
    // Mobile menu toggle
    const mobileMenuToggle = document.querySelector('.mobile-menu-toggle');
    const sidebar = document.querySelector('.neo-sidebar');
    
    if (mobileMenuToggle && sidebar) {
        mobileMenuToggle.addEventListener('click', function() {
            sidebar.classList.toggle('show');
        });
        
        // Close sidebar when clicking outside on mobile
        document.addEventListener('click', function(event) {
            if (window.innerWidth <= 768 && 
                !sidebar.contains(event.target) && 
                !mobileMenuToggle.contains(event.target) &&
                sidebar.classList.contains('show')) {
                sidebar.classList.remove('show');
            }
        });
    }
    
    // Smooth scrolling for navigation links
    const navLinks = document.querySelectorAll('.neo-nav a[href^="#"]');
    
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            if (this.getAttribute('href').startsWith('#')) {
                e.preventDefault();
                
                const targetId = this.getAttribute('href');
                const targetElement = document.querySelector(targetId);
                
                if (targetElement) {
                    // Close mobile menu if open
                    if (window.innerWidth <= 768 && sidebar) {
                        sidebar.classList.remove('show');
                    }
                    
                    // Smooth scroll to target
                    window.scrollTo({
                        top: targetElement.offsetTop - 20,
                        behavior: 'smooth'
                    });
                    
                    // Update URL without scrolling
                    history.pushState(null, null, targetId);
                }
            }
        });
    });
    
    // Active link highlighting
    function setActiveLink() {
        const sections = document.querySelectorAll('.neo-section');
        const scrollPosition = window.scrollY + 100;
        
        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            const sectionId = section.getAttribute('id');
            
            if (sectionId && scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
                navLinks.forEach(link => {
                    link.classList.remove('active');
                    if (link.getAttribute('href') === `#${sectionId}`) {
                        link.classList.add('active');
                    }
                });
            }
        });
    }
    
    window.addEventListener('scroll', setActiveLink);
    setActiveLink();
    
    // Animated entrance for feature boxes
    const featureBoxes = document.querySelectorAll('.feature-box');
    
    if ('IntersectionObserver' in window) {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.style.opacity = '1';
                    entry.target.style.transform = 'translateY(0)';
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1 });
        
        featureBoxes.forEach(box => {
            box.style.opacity = '0';
            box.style.transform = 'translateY(20px)';
            box.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
            observer.observe(box);
        });
    }
    
    // Code syntax highlighting with Prism.js if available
    if (window.Prism) {
        Prism.highlightAll();
    }
    
    // Add copy buttons to code blocks
    const codeBlocks = document.querySelectorAll('.code-block');
    
    codeBlocks.forEach(block => {
        const copyButton = document.createElement('button');
        copyButton.className = 'code-copy-btn';
        copyButton.textContent = 'Copy';
        block.appendChild(copyButton);
        
        copyButton.addEventListener('click', function() {
            const code = block.textContent.replace('Copy', '').trim();
            
            navigator.clipboard.writeText(code).then(() => {
                copyButton.textContent = 'Copied!';
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                }, 2000);
            }).catch(err => {
                console.error('Failed to copy code: ', err);
                copyButton.textContent = 'Error';
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                }, 2000);
            });
        });
    });
    
    // Dark mode toggle
    const themeToggle = document.querySelector('.theme-toggle');
    const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)');
    
    // Check for saved theme preference or use system preference
    const savedTheme = localStorage.getItem('theme');
    
    if (savedTheme === 'dark' || (!savedTheme && prefersDarkScheme.matches)) {
        document.body.classList.add('dark-mode');
        if (themeToggle) {
            themeToggle.textContent = '☀️';
            themeToggle.setAttribute('aria-label', 'Switch to light mode');
        }
    } else {
        if (themeToggle) {
            themeToggle.textContent = '🌙';
            themeToggle.setAttribute('aria-label', 'Switch to dark mode');
        }
    }
    
    if (themeToggle) {
        themeToggle.addEventListener('click', function() {
            document.body.classList.toggle('dark-mode');
            
            if (document.body.classList.contains('dark-mode')) {
                localStorage.setItem('theme', 'dark');
                themeToggle.textContent = '☀️';
                themeToggle.setAttribute('aria-label', 'Switch to light mode');
            } else {
                localStorage.setItem('theme', 'light');
                themeToggle.textContent = '🌙';
                themeToggle.setAttribute('aria-label', 'Switch to dark mode');
            }
        });
    }
    
    // Accordion functionality
    const accordionItems = document.querySelectorAll('.neo-accordion-item');
    
    accordionItems.forEach(item => {
        const header = item.querySelector('.neo-accordion-header');
        if (header) {
            header.addEventListener('click', function() {
                // Toggle current item
                item.classList.toggle('active');
                
                // Close other items (optional - for single open accordion)
                // accordionItems.forEach(otherItem => {
                //     if (otherItem !== item) {
                //         otherItem.classList.remove('active');
                //     }
                // });
            });
        }
    });
    
    // Search functionality
    const searchInput = document.querySelector('.neo-search');
    if (searchInput) {
        searchInput.addEventListener('input', function() {
            const searchTerm = this.value.toLowerCase();
            const sections = document.querySelectorAll('.neo-section');
            
            sections.forEach(section => {
                const sectionText = section.textContent.toLowerCase();
                const sectionTitle = section.querySelector('h2')?.textContent.toLowerCase() || '';
                
                if (sectionText.includes(searchTerm) || sectionTitle.includes(searchTerm)) {
                    section.style.display = 'block';
                } else {
                    section.style.display = 'none';
                }
            });
        });
    }
    
    // Skip link functionality for accessibility
    const skipLink = document.querySelector('.skip-link');
    if (skipLink) {
        skipLink.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                targetElement.setAttribute('tabindex', '-1');
                targetElement.focus();
            }
        });
    }
    
    // Add target="_blank" and rel="noopener noreferrer" to external links
    const links = document.querySelectorAll('a[href^="http"]');
    links.forEach(link => {
        if (!link.hasAttribute('target')) {
            link.setAttribute('target', '_blank');
            link.setAttribute('rel', 'noopener noreferrer');
        }
    });
}); 