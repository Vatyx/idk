class idk < Formula
    desc "diff tool with browser-based GUI"
    homepage "https://github.com/Vatyx/idk"
    url "https://api.github.com/Vatyx/idk/releases/latest/idk"
  
    bottle :unneeded
  
    def install
      bin.install "delta"
    end
  
    test do
      system "#{bin}/idk", "--version"
    end
  end